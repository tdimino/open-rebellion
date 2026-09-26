
bool __thiscall FUN_00573610(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  uint uVar3;
  undefined3 extraout_var_00;
  uint *puVar4;
  void *pvVar5;
  undefined3 extraout_var_01;
  int *this_00;
  bool bVar6;
  int *local_50;
  int iStack_4c;
  uint uStack_48;
  uint uStack_44;
  uint auStack_40 [4];
  void *local_30;
  undefined4 auStack_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064c620;
  local_c = ExceptionList;
  bVar6 = true;
  ExceptionList = &local_c;
  local_30 = this;
  if (*(int *)((int)this + 0x60) == 0) {
    ExceptionList = &local_c;
    iVar2 = FUN_00521880(this,2,param_1);
    bVar6 = iVar2 != 0;
  }
  if (*(int *)((int)this + 0x60) == 3) {
    local_50 = (int *)0x0;
    bVar1 = FUN_00521070(this,(int *)&local_50);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    this_00 = (int *)0x0;
    if (local_50 != (int *)0x0) {
      auStack_40[2] = 0x90;
      auStack_40[3] = 0x98;
      local_4 = 0;
      uVar3 = (**(code **)(*local_50 + 4))();
      if ((uVar3 < auStack_40[2]) || (auStack_40[3] <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      this_00 = local_50;
      if (!bVar1) {
        bVar6 = false;
        this_00 = (int *)0x0;
      }
    }
    iVar2 = 0;
    if (this_00 != (int *)0x0) {
      bVar1 = FUN_0050d5a0(this_00,10,*(uint *)((int)this + 0x24) >> 6 & 3,(int)param_1);
      if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar6)) {
        bVar6 = false;
      }
      else {
        bVar6 = true;
      }
      uVar3 = *(uint *)((int)this + 0x24) >> 6 & 3;
      if (uVar3 == 3) {
        uVar3 = 3;
      }
      else {
        uVar3 = 2 - (uVar3 != 1);
      }
      if (uVar3 == ((uint)this_00[9] >> 6 & 3)) {
        auStack_40[2] = 0x90;
        auStack_40[3] = 0x92;
        local_4 = 1;
        uVar3 = (**(code **)(*this_00 + 4))();
        if ((uVar3 < auStack_40[2]) || (iStack_4c = 1, auStack_40[3] <= uVar3)) {
          iStack_4c = 0;
        }
        local_4 = 0xffffffff;
        FUN_00619730();
        puVar4 = FUN_004025b0(this_00,auStack_40 + 2);
        local_4 = 2;
        auStack_40[1] = 0x90000109;
        if ((*puVar4 == 0x90000109) && (((byte)this_00[9] & 0xc0) == 0x80)) {
          iVar2 = 1;
        }
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        pvVar5 = FUN_00526cf0(auStack_2c,this_00,1);
        local_4 = 3;
        bVar1 = FUN_005131b0((int)pvVar5);
        local_4 = 0xffffffff;
        FUN_00526df0(auStack_2c);
        uStack_48 = 0;
        auStack_40[0] = 0;
        uStack_44 = 0;
        iVar2 = FUN_0055c940(iStack_4c,iVar2,CONCAT31(extraout_var_01,bVar1),(int *)&uStack_48,
                             auStack_40,&uStack_44);
        if ((iVar2 == 0) || (!bVar6)) {
          bVar6 = false;
        }
        else {
          bVar6 = true;
        }
        if (uStack_48 != 0) {
          iVar2 = FUN_00573170(local_30,this_00,uStack_48,auStack_40[0],uStack_44,param_1);
          if ((iVar2 == 0) || (!bVar6)) {
            bVar6 = false;
          }
          else {
            bVar6 = true;
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return bVar6;
}

