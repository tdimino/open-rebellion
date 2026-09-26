
uint __thiscall FUN_005444e0(void *this,void *param_1)

{
  bool bVar1;
  uint *puVar2;
  int iVar3;
  uint uVar4;
  uint local_18;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647408;
  local_c = ExceptionList;
  uVar4 = 1;
  if ((*(byte *)((int)this + 0x58) & 0x40) != 0) {
    ExceptionList = &local_c;
    FUN_004ece30(&local_18);
    local_4 = 0;
    uVar4 = FUN_00543690(&local_14,&local_10,&local_18);
    if ((local_14 != 0) && (local_10 != 0)) {
      puVar2 = FUN_004ece40(&local_18);
      if (puVar2 != (uint *)0x0) {
        iVar3 = FUN_005439e0(local_14);
        if (iVar3 != 0) {
          iVar3 = FUN_00543d20(this,1,param_1);
          if ((iVar3 == 0) || (uVar4 == 0)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          iVar3 = FUN_00543d20(this,0,param_1);
          if ((iVar3 == 0) || (!bVar1)) {
            uVar4 = 0;
          }
          else {
            uVar4 = 1;
          }
        }
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return uVar4;
}

