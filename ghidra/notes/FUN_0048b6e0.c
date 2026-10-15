
void __thiscall FUN_0048b6e0(int param_1,int param_2,undefined4 param_3,int param_4)

{
  undefined2 uVar1;
  int iVar2;
  undefined4 uVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00635198;
  pvStack_c = ExceptionList;
  if (((param_2 == 0x231) && (*(int *)(param_1 + 100) == 0)) && (param_4 != 0)) {
    ExceptionList = &pvStack_c;
    *(undefined4 *)(param_1 + 100) = 1;
    *(undefined4 *)(param_1 + 0x40) = 1;
    iVar2 = FUN_004f3e50(*(undefined4 *)(param_1 + 0x20));
    if ((*(byte *)(iVar2 + 0xb0) & 8) == 0) {
      *(undefined2 *)(param_1 + 0x2e) = 0x42a;
      *(undefined2 *)(param_1 + 0x30) = 0x479;
      uVar3 = 0x123;
    }
    else {
      *(undefined2 *)(param_1 + 0x2e) = 0x42b;
      *(undefined2 *)(param_1 + 0x30) = 0x47a;
      uVar3 = 0x139;
    }
    uVar1 = FUN_004c4990(uVar3);
    *(undefined2 *)(param_1 + 0x32) = uVar1;
    *(undefined4 *)(param_1 + 0x28) = 0x1b;
    param_2 = 0x35000281;
    uStack_4 = 0;
    FUN_004f26d0(&param_2);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    FUN_0060b9d0(param_1 + 0x44,0x7200,0,0,0,0);
    FUN_0060b9d0(param_1 + 0x50,0x7201,0,0,0,0);
  }
  ExceptionList = pvStack_c;
  return;
}

