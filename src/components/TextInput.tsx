type myCustomPrompts = {
  title: string;
  id: string;
  password: boolean;
};
export default function TextInput(prompts: myCustomPrompts) {
  return (
    <div className="test-input">
      {prompts.title !== "" && (
        <h3 className="test-input-title">{prompts.title}</h3>
      )}

      <input
        className="test-input-field"
        type={prompts.password ? "password" : "text"}
        name={prompts.title}
        id={prompts.id}
      />
    </div>
  );
}
