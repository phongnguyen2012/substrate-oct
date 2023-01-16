import { useState } from 'react';

export default function App() {
  const [updated, setUpdated] = useState(0);

  const handleKeyDown = (event: any) => {
    if (event.key === 'Enter') {
      setUpdated(event.target.value);
    }
  };

  return (
    <div>
      <input
        type="number" pattern="[0-9]*"
        id="message"
        name="message"
        onKeyDown={handleKeyDown}
      />

      <h2>Updated: {updated}</h2>
      {updated}
    </div>
  );
}