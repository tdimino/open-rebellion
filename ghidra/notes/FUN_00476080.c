
undefined4 __fastcall FUN_00476080(int param_1)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 uVar5;
  undefined4 uStack_1c;
  
  uVar5 = 0;
  uVar3 = *(uint *)(param_1 + 0x2c) >> 0x18;
  if ((uVar3 < 8) || (0xf < uVar3)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  uStack_1c = 0x4760c0;
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x4c);
    FUN_004f26d0(&uStack_1c,(undefined4 *)(param_1 + 0x2c));
    pvVar4 = FUN_00419e40((void *)(iVar1 + 0x78));
    if (pvVar4 != (void *)0x0) {
      uVar3 = *(uint *)((int)pvVar4 + 0x38);
      *(uint *)((int)pvVar4 + 0x38) = uVar3 | 4;
      *(uint *)((int)pvVar4 + 0x38) = uVar3 & 0xfffffff7 | 4;
      *(undefined4 *)((int)pvVar4 + 0x30) = 0;
      *(undefined4 *)((int)pvVar4 + 0x34) = 0;
    }
    uStack_1c = 0x476100;
    FUN_004ece80((undefined4 *)(param_1 + 0x2c));
    uVar5 = 1;
  }
  return uVar5;
}

