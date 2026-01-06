import { useState, useRef, useEffect } from "react";
import { useChatStore } from "../store/chat";
import { useAgentsStore } from "../store/agents";

export function ChatInterface() {
  const [input, setInput] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { messages, isStreaming, streamingMessage, addMessage, setIsStreaming } = useChatStore();
  const { selectedAgent } = useAgentsStore();

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, streamingMessage]);

  const sendMessage = async () => {
    if (!input.trim() || !selectedAgent) return;

    const userMessage = input.trim();
    setInput("");

    // Add user message
    addMessage({
      role: "user",
      content: userMessage,
      agent: selectedAgent,
    });

    setIsStreaming(true);

    try {
      const response = await fetch(`/api/agents/${selectedAgent}/chat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          messages: [
            {
              role: "user",
              content: userMessage,
            },
          ],
        }),
      });

      if (!response.ok) throw new Error("Failed to send message");

      const data = await response.json();

      addMessage({
        role: "assistant",
        content: data.response || data.content || "No response",
        agent: selectedAgent,
      });
    } catch (error) {
      console.error("Error sending message:", error);
      addMessage({
        role: "assistant",
        content: "Sorry, I encountered an error. Please make sure the VoltAgent server is running on port 3141.",
        agent: selectedAgent,
      });
    } finally {
      setIsStreaming(false);
    }
  };

  return (
    <div className="flex flex-col h-[600px]">
      {/* Messages */}
      <div className="flex-1 overflow-y-auto space-y-4 mb-4">
        {messages.length === 0 && !selectedAgent && (
          <div className="text-center text-gray-500 mt-20">
            <p className="text-lg">👋 Welcome to GopenPal!</p>
            <p className="mt-2">Select an agent above to start chatting</p>
          </div>
        )}

        {messages.length === 0 && selectedAgent && (
          <div className="text-center text-gray-500 mt-20">
            <p className="text-lg">Ready to chat!</p>
            <p className="mt-2">Ask me anything</p>
          </div>
        )}

        {messages.map((message) => (
          <div
            key={message.id}
            className={`flex ${message.role === "user" ? "justify-end" : "justify-start"}`}
          >
            <div
              className={`max-w-[80%] rounded-lg px-4 py-2 ${
                message.role === "user" ? "bg-blue-500 text-white" : "bg-gray-100 text-gray-900"
              }`}
            >
              <p className="whitespace-pre-wrap">{message.content}</p>
              <p className="text-xs mt-1 opacity-70">
                {new Date(message.timestamp).toLocaleTimeString()}
              </p>
            </div>
          </div>
        ))}

        {isStreaming && streamingMessage && (
          <div className="flex justify-start">
            <div className="max-w-[80%] rounded-lg px-4 py-2 bg-gray-100 text-gray-900">
              <p className="whitespace-pre-wrap">{streamingMessage}</p>
              <div className="flex space-x-1 mt-2">
                <div className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                <div
                  className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"
                  style={{ animationDelay: "0.1s" }}
                ></div>
                <div
                  className="w-2 h-2 bg-gray-400 rounded-full animate-bounce"
                  style={{ animationDelay: "0.2s" }}
                ></div>
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="flex space-x-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === "Enter" && !e.shiftKey && sendMessage()}
          placeholder={selectedAgent ? "Type your message..." : "Select an agent to start chatting"}
          disabled={!selectedAgent || isStreaming}
          className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
        />
        <button
          onClick={sendMessage}
          disabled={!selectedAgent || isStreaming || !input.trim()}
          className="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
        >
          Send
        </button>
      </div>
    </div>
  );
}
