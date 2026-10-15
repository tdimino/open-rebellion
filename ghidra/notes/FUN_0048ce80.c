
void __thiscall FUN_0048ce80(int param_1,int param_2,undefined4 param_3,int param_4)

{
  undefined2 uVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puStack_8 = &LAB_00635488;
  pvStack_c = ExceptionList;
  if (((param_2 == 0x230) && (*(int *)(param_1 + 100) == 0)) && (param_4 != 0)) {
    ExceptionList = &pvStack_c;
    *(undefined4 *)(param_1 + 100) = 1;
    if (*(int *)(param_1 + 0x20) == 2) {
      *(undefined4 *)(param_1 + 0x40) = 1;
    }
    if (*(int *)(param_1 + 0x40) != 0) {
      param_2 = 0x34000280;
      uStack_4 = 0;
      FUN_004f26d0(&param_2);
      uStack_4 = 0xffffffff;
      FUN_00619730();
      *(undefined2 *)(param_1 + 0x2e) = 0x42c;
      *(undefined4 *)(param_1 + 0x28) = 0x1a;
      *(undefined2 *)(param_1 + 0x30) = 0x47b;
      uVar1 = FUN_004c4990(0x130);
      *(undefined2 *)(param_1 + 0x32) = uVar1;
      FUN_0060b9d0(param_1 + 0x44,0x71c8,0,0,0,0);
      FUN_0060b9d0(param_1 + 0x50,0x71c9,0,0,0,0);
    }
  }
  ExceptionList = pvStack_c;
  return;
}

