
void __fastcall FUN_00438980(int param_1)

{
  uint *puVar1;
  bool bVar2;
  int iVar3;
  undefined4 *this;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d728;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar3 = FUN_0060c960(*(int *)(param_1 + 0x114));
  if (*(int *)(param_1 + 0x144) == 0) {
    ExceptionList = local_c;
    return;
  }
  if (iVar3 == 0) {
    ExceptionList = local_c;
    return;
  }
  puVar1 = *(uint **)(iVar3 + 0x54);
  FUN_00438d30(param_1);
  bVar2 = true;
  if (*(int *)(param_1 + 0x148) < 1) {
    *(undefined4 *)(param_1 + 0x148) = 1;
  }
  else if (0xff < *(int *)(param_1 + 0x148)) {
    *(undefined4 *)(param_1 + 0x148) = 0xff;
  }
  *(undefined4 *)(*(int *)(param_1 + 0x144) + 0x48) = *(undefined4 *)(param_1 + 0x148);
  FUN_004f26d0((void *)(*(int *)(param_1 + 0x144) + 0x44),puVar1);
  uVar4 = *puVar1 >> 0x18;
  if ((uVar4 < 0x20) || (0x2f < uVar4)) {
    bVar2 = false;
  }
  FUN_00619730();
  if (bVar2) {
    this = &DAT_006b289c;
  }
  else {
    if ((uVar4 < 0x14) || (0x1f < uVar4)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      this = &DAT_006b28a4;
    }
    else {
      local_4 = 0;
      if ((uVar4 < 0x10) || (0x13 < uVar4)) {
        if ((uVar4 < 0x3c) || (0x3f < uVar4)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) goto LAB_00438ae1;
        bVar2 = false;
      }
      else {
LAB_00438ae1:
        bVar2 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (!bVar2) goto LAB_00438b07;
      this = &DAT_006b28a0;
    }
  }
  FUN_004f26d0(this,puVar1);
LAB_00438b07:
  FUN_0041ce20(*(int **)(param_1 + 0x144),0);
  ExceptionList = local_c;
  return;
}

