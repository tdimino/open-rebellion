
bool __thiscall FUN_005442f0(void *this,void *param_1)

{
  uint uVar1;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  uint local_1c;
  int local_18;
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006473c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  local_4 = 0;
  uVar1 = FUN_005435f0(&local_18,local_14,&local_1c);
  bVar4 = uVar1 != 0;
  if ((local_18 == 0) || (local_14[0] == 0)) goto LAB_005443c3;
  puVar2 = FUN_004ece40(&local_1c);
  if (puVar2 == (uint *)0x0) goto LAB_005443c3;
  if (((((*(byte *)(local_18 + 0xac) & 1) == 0) || ((*(uint *)(local_18 + 0x50) & 1) == 0)) ||
      ((*(byte *)(local_18 + 0x78) & 0xc0) != 0)) || ((*(uint *)(local_18 + 0x50) & 0x1000) != 0)) {
LAB_005443a3:
    iVar3 = 0;
  }
  else {
    local_14[1] = 0x98000481;
    FUN_00619730();
    if (local_1c != 0x98000481) goto LAB_005443a3;
    iVar3 = 1;
  }
  iVar3 = FUN_00543c40(this,iVar3,param_1);
  if ((iVar3 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
LAB_005443c3:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar4;
}

