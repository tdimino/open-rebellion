
void __fastcall FUN_0042d570(int param_1)

{
  HCURSOR hCursor;
  
  hCursor = LoadCursorA((HINSTANCE)0x0,(LPCSTR)0x7f02);
  *(HCURSOR *)(param_1 + 0x46c) = hCursor;
  SetCursor(hCursor);
  if (*(int **)(param_1 + 0x134) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x134) + 0x10))();
  }
  if (*(int **)(param_1 + 0x138) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x138) + 0x10))();
  }
  if (*(int **)(param_1 + 0x13c) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x13c) + 0x10))();
  }
  if (*(int **)(param_1 + 0x140) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x140) + 0x10))();
  }
  if (*(int **)(param_1 + 0x144) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x144) + 0x10))();
  }
  if (*(int **)(param_1 + 0x148) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x148) + 0x10))();
  }
  if (*(int **)(param_1 + 0x14c) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x14c) + 0x10))();
  }
  if (*(int **)(param_1 + 0x150) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x150) + 0x10))();
  }
  if (*(int **)(param_1 + 0x248) != (int *)0x0) {
    (**(code **)(**(int **)(param_1 + 0x248) + 0x10))();
  }
  return;
}

