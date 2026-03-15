
bool FUN_00547f60(void *param_1,uint param_2)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  uint uVar3;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  int iVar4;
  int *local_a4;
  uint local_a0;
  uint local_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 auStack_7c [4];
  int *piStack_6c;
  void *pvStack_68;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647a1e;
  local_c = ExceptionList;
  bVar2 = true;
  ExceptionList = &local_c;
  bVar1 = FUN_00520e40((int)param_1);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    local_a4 = (int *)0x0;
    bVar2 = FUN_00520ad0((int)param_1);
    if (CONCAT31(extraout_var_00,bVar2) == 0) {
      bVar2 = FUN_00520f40(param_1,(int *)&local_a4);
      iVar4 = CONCAT31(extraout_var_02,bVar2);
    }
    else {
      bVar2 = FUN_00521070(param_1,(int *)&local_a4);
      iVar4 = CONCAT31(extraout_var_01,bVar2);
    }
    bVar2 = iVar4 != 0;
    if (local_a4 != (int *)0x0) {
      local_a0 = 0x90;
      local_9c = 0x98;
      local_4 = 0;
      uVar3 = (**(code **)(*local_a4 + 4))();
      if ((uVar3 < local_a0) || (local_9c <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      piStack_6c = local_a4;
      if (bVar1) {
        FUN_005897c0(auStack_7c);
        local_4 = 1;
        pvStack_68 = param_1;
        bVar1 = FUN_005898f0(auStack_7c,param_2);
        if ((CONCAT31(extraout_var_03,bVar1) == 0) || (!bVar2)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        bVar1 = FUN_00520e40((int)param_1);
        if (CONCAT31(extraout_var_04,bVar1) != 0) {
          iVar4 = -1;
          uVar3 = *(uint *)((int)param_1 + 0x24) >> 6 & 3;
          if (uVar3 == 1) {
            iVar4 = 2;
          }
          else if (uVar3 == 2) {
            iVar4 = 1;
          }
          if ((iVar4 == -1) || (!bVar2)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          if (iVar4 != -1) {
            FUN_004fd450(&uStack_98,param_2);
            local_4._0_1_ = 2;
            uStack_94 = 1;
            uStack_90 = 3;
            FUN_004f9510(param_1,iVar4,&uStack_98);
            local_4 = CONCAT31(local_4._1_3_,1);
            FUN_004fd4d0(&uStack_98);
          }
        }
        local_4 = 0xffffffff;
        FUN_00589870(auStack_7c);
      }
    }
  }
  ExceptionList = local_c;
  return bVar2;
}

