import React from 'react';
import { renderToKivy } from './kivy-reconciler';

const View: React.ElementType = 'BoxLayout';
const Label: React.ElementType = 'Label';
const Button: React.ElementType = 'Button';
const TextInput: React.ElementType = 'TextInput';

const App = () => {
  const [draft, setDraft] = React.useState('');
  const [items, setItems] = React.useState<Array<{ id: number; text: string; done: boolean }>>([
    { id: 1, text: 'Set up Rust + Python bridge', done: true },
    { id: 2, text: 'Bundle React UI into app.bundle.js', done: true },
    { id: 3, text: 'Ship TODO app via Python runtime', done: false },
  ]);

  const addItem = () => {
    const text = draft.trim();
    if (!text) return;
    setItems((prev) => [...prev, { id: Date.now(), text, done: false }]);
    setDraft('');
  };

  const toggleItem = (id: number) => {
    setItems((prev) => prev.map((item) => (item.id === id ? { ...item, done: !item.done } : item)));
  };

  return (
    <View orientation="vertical" padding={[20, 20, 20, 20]} spacing={12}>
      <Label text="Kivy JSX TODO" font_size={28} color={[0.2, 0.9, 1, 1]} />

      <View orientation="horizontal" spacing={8}>
        <TextInput
          text={draft}
          hint_text="Add new todo..."
          multiline={false}
          on_text={(value: string) => setDraft(value)}
        />
        <Button text="Add" on_press={addItem} />
      </View>

      <View orientation="vertical" spacing={6}>
        {items.map((item) => (
          <Button
            key={item.id}
            text={`${item.done ? '[x]' : '[ ]'} ${item.text}`}
            on_press={() => toggleItem(item.id)}
          />
        ))}
      </View>
    </View>
  );
};

renderToKivy(<App />);
