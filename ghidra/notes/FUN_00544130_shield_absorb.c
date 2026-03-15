
bool __thiscall FUN_00544130(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  uint *puVar3;
  int iVar4;
  int *piVar5;
  bool bVar6;
  void *local_24;
  uint local_20;
  int local_1c;
  undefined1 local_18 [4];
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006473a8;
  local_c = ExceptionList;
  if ((*(byte *)((int)this + 0x58) & 0x40) == 0) {
    return true;
  }
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  local_4 = 0;
  uVar2 = FUN_005434b0(&local_24,&local_1c,&local_20);
  bVar6 = uVar2 != 0;
  if ((((local_24 == (void *)0x0) || (local_1c == 0)) ||
      (puVar3 = FUN_004ece40(&local_20), puVar3 == (uint *)0x0)) ||
     (iVar4 = FUN_005439b0((int)local_24), iVar4 == 0)) goto LAB_005442bd;
  if ((*(byte *)((int)local_24 + 0x78) & 0x80) == 0) {
    iVar4 = FUN_00543bd0(this,1,param_1);
    if ((iVar4 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    iVar4 = FUN_00543bd0(this,0,param_1);
    if ((iVar4 != 0) && (bVar6)) {
      bVar6 = true;
      goto LAB_005442bd;
    }
  }
  else {
    puVar3 = (uint *)FUN_0042d170(local_24,local_18);
    local_14 = 0x71;
    local_10 = 0x72;
    local_4._0_1_ = 1;
    if ((*puVar3 >> 0x18 < 0x71) || (0x71 < *puVar3 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4._0_1_ = 0;
    FUN_00619730();
    if (!bVar1) goto LAB_005442bd;
    puVar3 = (uint *)FUN_0042d170(local_24,local_18);
    local_4._0_1_ = 2;
    piVar5 = (int *)FUN_00505970(puVar3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if ((piVar5 == (int *)0x0) || (piVar5[0x1a] != 8)) goto LAB_005442bd;
    iVar4 = (**(code **)(*piVar5 + 0x1d8))(param_1);
    if ((iVar4 != 0) && (bVar6)) {
      bVar6 = true;
      goto LAB_005442bd;
    }
  }
  bVar6 = false;
LAB_005442bd:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar6;
}

