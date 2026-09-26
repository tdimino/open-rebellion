
bool __thiscall FUN_00522280(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  bool bVar3;
  int local_58;
  int *local_54;
  uint local_50;
  uint uStack_4c;
  int local_48;
  int local_44;
  void *local_40;
  undefined4 auStack_3c [11];
  int *piStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643970;
  pvStack_c = ExceptionList;
  bVar3 = true;
  local_44 = 0;
  ExceptionList = &pvStack_c;
  local_40 = this;
  bVar1 = FUN_00520b20((int)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    local_50 = 1;
    iVar2 = FUN_00522a90(this,&local_50);
    bVar3 = iVar2 != 0;
    if (local_50 != 0) {
      local_54 = (int *)0x0;
      bVar1 = FUN_00521050(this,(int *)&local_54);
      if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar3)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      if (local_54 != (int *)0x0) {
        FUN_004ece30(&local_58);
        local_4 = 0;
        iVar2 = (**(code **)(*local_54 + 0xc))(&local_58);
        if ((iVar2 == 0) || (!bVar3)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        iVar2 = FUN_00521c10(this,&local_58,param_1);
        if ((iVar2 == 0) || (!bVar3)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      local_58 = 1;
      local_48 = 1;
      iVar2 = thunk_FUN_00506e60();
      if ((iVar2 == 0) || (!bVar3)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      if (iVar2 != 0) {
        FUN_00525bb0(auStack_3c,this);
        local_4 = 1;
        FUN_00525930((int)auStack_3c);
        while (piStack_10 != (int *)0x0) {
          uStack_4c = 1;
          if ((*(byte *)(piStack_10 + 0x14) & 0x20) != 0) {
            bVar1 = FUN_00556620(piStack_10,&uStack_4c);
            if ((CONCAT31(extraout_var_01,bVar1) == 0) || (!bVar3)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
          }
          if (uStack_4c == 0) {
            local_58 = 0;
          }
          else {
            local_48 = 0;
          }
          FUN_005258f0((int)auStack_3c);
          this = local_40;
        }
        if ((local_58 == 0) && (local_48 == 0)) {
          local_58 = 1;
        }
        local_4 = 0xffffffff;
        FUN_00525c50(auStack_3c);
      }
      if ((local_58 != 0) && (bVar1 = FUN_00520e40((int)this), CONCAT31(extraout_var_02,bVar1) == 0)
         ) {
        local_44 = 1;
      }
    }
  }
  if (local_44 != 0) {
    iVar2 = FUN_00522130(this,1,param_1);
    if ((iVar2 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
  }
  ExceptionList = pvStack_c;
  return bVar3;
}

