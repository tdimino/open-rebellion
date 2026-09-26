
bool __thiscall FUN_00544030(void *this,void *param_1)

{
  uint uVar1;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  uint local_18;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647378;
  local_c = ExceptionList;
  bVar4 = true;
  if (((*(uint *)((int)this + 0x58) & 1) != 0) && ((*(uint *)((int)this + 0x58) & 0x40) == 0)) {
    ExceptionList = &local_c;
    FUN_004ece30(&local_18);
    local_4 = 0;
    uVar1 = FUN_005434b0(&local_14,&local_10,&local_18);
    bVar4 = uVar1 != 0;
    if ((local_14 != 0) && (local_10 != 0)) {
      puVar2 = FUN_004ece40(&local_18);
      if (puVar2 != (uint *)0x0) {
        iVar3 = FUN_005439e0(local_14);
        if (iVar3 != 0) {
          iVar3 = FUN_00543b60(this,1,param_1);
          if ((iVar3 == 0) || (!bVar4)) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
          iVar3 = FUN_00543b60(this,0,param_1);
          if ((iVar3 == 0) || (!bVar4)) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar4;
}

