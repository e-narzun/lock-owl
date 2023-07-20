type customInput = {
  path: string;
};

const PasswordModal = (props: customInput) => {
  return (
    <div className="modal-background">
      <div className="modal-wrapper">
        <input type="text" className="password-input" />
        <button>Decrypt file</button>
        <button>Cancel</button>
      </div>
    </div>
  );
};

export default PasswordModal;
