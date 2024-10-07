import React, { useState } from 'react';
import styled from 'styled-components';

const CreatePoolModal = ({ isOpen, onClose }) => {
  if (!isOpen) return null;

  return (
    <Overlay>
      <ModalContainer>
        <Header>
          <h2>Swap Details</h2>
          <CloseButton onClick={onClose}>&times;</CloseButton>
        </Header>
        <Content>
          <p>You can swap directly without depositing, because you have sufficient balance in the Swap pool.</p>
          
          <Step>
            <StepHeader>
              <StepNumber>1</StepNumber>
              <StepTitle>Transfer ICP</StepTitle>
            </StepHeader>
          </Step>

          <Step active>
            <StepHeader>
              <StepNumber>2</StepNumber>
              <StepTitle>Deposit ICP</StepTitle>
            </StepHeader>
            <StepContent>
              <InputContainer>
                <label>Amount</label>
                <input type="number" placeholder="0.001" />
              </InputContainer>
              <InputContainer>
                <label>Canister Id</label>
                <input type="text" value="ryjl3-tyaaa-aaaaa-aaaba-cai" readOnly />
              </InputContainer>
            </StepContent>
          </Step>

          <Step>
            <StepHeader>
              <StepNumber>3</StepNumber>
              <StepTitle>Swap ICP to KAST</StepTitle>
            </StepHeader>
          </Step>

          <Step>
            <StepHeader>
              <StepNumber>4</StepNumber>
              <StepTitle>Withdraw KAST</StepTitle>
            </StepHeader>
          </Step>
        </Content>
      </ModalContainer>
    </Overlay>
  );
};

// Styled Components
const Overlay = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
`;

const ModalContainer = styled.div`
  background: #1a1a2e;
  padding: 20px;
  border-radius: 12px;
  width: 400px;
  max-width: 90%;
  color: #fff;
  box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.2);
`;

const Header = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #444;
  padding-bottom: 10px;
`;

const CloseButton = styled.button`
  background: transparent;
  border: none;
  color: #fff;
  font-size: 24px;
  cursor: pointer;
`;

const Content = styled.div`
  margin-top: 20px;
`;

const Step = styled.div`
  margin-bottom: 15px;
  padding: 10px;
  border-radius: 8px;
  background: ${(props) => (props.active ? "#16213e" : "#0f3460")};
  color: ${(props) => (props.active ? "#00ffb9" : "#fff")};
`;

const StepHeader = styled.div`
  display: flex;
  align-items: center;
`;

const StepNumber = styled.div`
  background: #00ffb9;
  color: #000;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  margin-right: 10px;
`;

const StepTitle = styled.h3`
  font-size: 18px;
`;

const StepContent = styled.div`
  margin-top: 10px;
  padding-left: 34px;
`;

const InputContainer = styled.div`
  margin-bottom: 10px;

  label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
  }

  input {
    width: 100%;
    padding: 8px;
    border: none;
    border-radius: 4px;
  }
`;

export default CreatePoolModal;
