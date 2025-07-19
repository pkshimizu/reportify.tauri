# 実装パターンとベストプラクティス

## Tauri-React通信パターン

### コマンド定義パターン
```rust
// src-tauri/src/lib.rs
#[tauri::command]
fn command_name(param: &str) -> Result<String, String> {
    // 実装
    Ok(result)
}

// 登録
.invoke_handler(tauri::generate_handler![command_name])
```

### フロントエンド呼び出しパターン
```typescript
// src/hooks/useTauriCommand.ts
import { invoke } from "@tauri-apps/api/core";

export const useTauriCommand = () => {
  const callCommand = async (name: string) => {
    try {
      return await invoke("command_name", { param: name });
    } catch (error) {
      console.error("Command failed:", error);
      throw error;
    }
  };
  return { callCommand };
};
```

## エラーハンドリングパターン

### Rustサイドでのエラーハンドリング
```rust
#[tauri::command]
fn safe_operation() -> Result<String, String> {
    match risky_operation() {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Operation failed: {}", e))
    }
}
```

### Reactサイドでのエラーハンドリング
```typescript
const handleTauriCommand = async () => {
  try {
    const result = await invoke("safe_operation");
    setData(result);
  } catch (error) {
    setError(error as string);
  }
};
```

## 状態管理パターン

### React Context利用例
```typescript
// contexts/AppContext.tsx
interface AppState {
  isLoading: boolean;
  data: any;
  error: string | null;
}

const AppContext = createContext<AppState | null>(null);
```

## 更新履歴
- 2025-07-19: 初期テンプレート作成