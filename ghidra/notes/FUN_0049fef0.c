
void __thiscall FUN_0049fef0(void *this,HDC param_1)

{
  HDC pHVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  undefined *puVar5;
  char *pcVar6;
  void *this_00;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006374e0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_24);
  local_4 = 0;
  piVar2 = FUN_004a25c0(this);
  pHVar1 = param_1;
  if (piVar2 == (int *)0x0) {
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_24);
    ExceptionList = local_c;
    return;
  }
  if (*(int **)((int)this + 0x170) != (int *)0x0) {
    iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x170));
    iVar4 = FUN_005fc0f0(*(int **)((int)this + 0x170));
    FUN_005fc140(*(void **)((int)this + 0x170),pHVar1,'@',0xcc0020,(0x7a - iVar3) / 2 + 0x6c,
                 (0x32 - iVar4) / 2 + 0x25,0,0,0,0);
  }
  FUN_00601ce0(*(void **)((int)this + 0x174),pHVar1);
  FUN_00601ce0(*(void **)((int)this + 0x178),pHVar1);
  if (*(int *)(*(int *)((int)this + 0x14c) + 0xb8) == *(int *)((int)this + 0x18)) {
    iVar3 = *(int *)((int)this + 300);
    iVar4 = *(int *)((int)this + 0x128);
    this_00 = *(void **)((int)this + 0x16c);
  }
  else {
    iVar3 = *(int *)((int)this + 300);
    iVar4 = *(int *)((int)this + 0x128);
    this_00 = *(void **)((int)this + 0x168);
  }
  FUN_005fc140(this_00,pHVar1,'@',0xcc0020,iVar4,iVar3,0,0,0,0);
  param_1 = (HDC)CONCAT22(DAT_0065d424,0x8525);
  FUN_005f3010(local_24,&param_1);
  puVar5 = FUN_004f62d0((int)piVar2);
  pcVar6 = (char *)FUN_00583c40((int)puVar5);
  FUN_005f35b0(local_18,pcVar6);
  local_4._0_1_ = 1;
  FUN_005f30d0(local_24,(int)local_18);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  *(undefined4 *)((int)this + 0x180) = 0x10;
  piVar2 = (int *)((int)this + 0x17c);
  *piVar2 = *(int *)((int)this + 0x164) * -3 + 0xeb;
  FUN_00601b30(piVar2,*(int *)((int)this + 0x164) + *(int *)((int)this + 0x128),
               *(int *)((int)this + 300));
  pcVar6 = (char *)FUN_00583c40((int)local_24);
  FUN_00601aa0(piVar2,pcVar6);
  FUN_00403e90(piVar2,0x24);
  *(undefined4 *)((int)this + 0x1a8) = 3;
  FUN_00601c60(piVar2,5);
  FUN_00601ce0(piVar2,pHVar1);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  ExceptionList = local_c;
  return;
}

