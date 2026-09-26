
void __fastcall FUN_0042d780(int param_1)

{
  undefined4 uVar1;
  char *pcVar2;
  undefined4 local_28 [3];
  RECT local_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c7e0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_28);
  local_4 = 0;
  FUN_005f35b0(&local_1c,s_Current_Latency__006a86f0);
  local_4._0_1_ = 1;
  FUN_005f30d0(local_28,(int)&local_1c);
  uVar1 = FUN_005971a0();
  FUN_005f31f0(local_28,uVar1);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(&local_1c.left);
  pcVar2 = (char *)FUN_00583c40((int)local_28);
  FUN_00601aa0((void *)(param_1 + 0x214),pcVar2);
  local_1c.left = *(LONG *)(param_1 + 0x228);
  local_1c.top = *(LONG *)(param_1 + 0x22c);
  local_1c.right = *(LONG *)(param_1 + 0x230);
  local_1c.bottom = *(LONG *)(param_1 + 0x234);
  InvalidateRect(*(HWND *)(param_1 + 0x18),&local_1c,0);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_28);
  ExceptionList = pvStack_c;
  return;
}

