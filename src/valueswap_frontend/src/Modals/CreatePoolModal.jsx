// CreatePoolModal.js
import React from 'react';

const CreatePoolModal = ({ isOpen, onClose, stepData, onSwap }) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      <div className="p-6 bg-gray-900 rounded-lg shadow-lg text-white max-w-sm mx-auto relative">
        <button
          className="absolute top-2 right-2 text-gray-400 hover:text-gray-300"
          onClick={onClose}
        >
          &times;
        </button>
        <h2 className="text-xl font-semibold mb-4">Swap Details</h2>
        <p className="text-gray-400 mb-6">
          You can swap directly without depositing, because you have sufficient balance in the Swap pool.
        </p>

        {stepData.map((step, index) => (
          <div key={index} className="mb-4">
            <div className="flex items-center mb-2">
              <div className={`w-6 h-6 flex items-center justify-center rounded-full ${step.completed ? 'bg-green-500' : 'bg-gray-600'}`}>
                {step.completed && <span className="text-white">✓</span>}
              </div>
              <h3 className="ml-3 font-medium">{step.title}</h3>
            </div>

            {step.input && (
              <div className="pl-9">
                <label className="block text-sm text-gray-400">Amount</label>
                <input
                  type="number"
                  className="w-full p-2 mt-1 bg-gray-800 rounded-md border border-gray-700 text-white"
                  placeholder="0.001"
                  value={step.amount}
                  readOnly // Making the field read-only
                />
                <label className="block text-sm text-gray-400 mt-2">Canister Id</label>
                <input
                  type="text"
                  className="w-full p-2 mt-1 bg-gray-800 rounded-md border border-gray-700 text-white"
                  value={step.canisterId}
                  readOnly // Making the field read-only
                />
              </div>
            )}
          </div>
        ))}

        <button
          className="w-full p-3 mt-4 bg-blue-600 hover:bg-blue-500 rounded-lg font-semibold"
          onClick={onSwap}
        >
          Proceed with Swap
        </button>
      </div>
    </div>
  );
};

export default CreatePoolModal;
