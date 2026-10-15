
void __thiscall FUN_0048d470(int param_1,int param_2,uint *param_3,undefined4 param_4)

{
  int iVar1;
  bool bVar2;
  int iVar3;
  uint *puVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00635598;
  pvStack_c = ExceptionList;
  if (param_2 == 0x305) {
    ExceptionList = &pvStack_c;
    iVar3 = FUN_004f2d10(*(undefined4 *)(param_1 + 0x20),param_4);
    iVar1 = *(int *)(iVar3 + 0x1c);
    bVar2 = false;
    iVar3 = 0;
    if (iVar1 != 0) {
      puVar4 = (uint *)FUN_004025b0(&param_4);
      uStack_4 = 0;
      if ((0x8f < *puVar4 >> 0x18) && (*puVar4 >> 0x18 < 0x98)) {
        bVar2 = true;
      }
      FUN_00619730();
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar2) {
        iVar3 = iVar1;
      }
    }
    if (((*param_3 >> 0x18 < 0x18) || (0x1b < *param_3 >> 0x18)) || (iVar3 == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      *(undefined4 *)(param_1 + 0x40) = 1;
      *(undefined4 *)(param_1 + 0x28) = 0x15;
      FUN_0060b9d0(param_1 + 0x44,0x71bd,0,0,0,0);
      if (iVar3 == 0) {
        iVar3 = 0;
      }
      else {
        iVar3 = iVar3 + 0x30;
      }
      FUN_0060b9d0(param_1 + 0x50,0x71be,iVar3,0,0,0);
      *(undefined4 *)(param_1 + 0x5c) = 0x72;
    }
  }
  else {
    ExceptionList = &pvStack_c;
    FUN_004c5020(param_2,param_3,param_4);
  }
  ExceptionList = pvStack_c;
  return;
}

