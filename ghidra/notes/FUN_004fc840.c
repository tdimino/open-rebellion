
uint __thiscall FUN_004fc840(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint uVar2;
  int iVar3;
  uint uVar4;
  void *pvVar5;
  int *local_54;
  uint local_50;
  uint local_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640218;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  uVar4 = CONCAT31(extraout_var,bVar1);
  pvVar5 = (void *)0x0;
  if (uVar4 != 0) {
    pvVar5 = (void *)FUN_00539fd0(this,0);
    uVar4 = (uint)(pvVar5 != (void *)0x0);
  }
  local_54 = (int *)0x0;
  if (uVar4 != 0) {
    bVar1 = FUN_004f6b50(pvVar5,(int *)&local_54);
    uVar4 = CONCAT31(extraout_var_00,bVar1);
    if (((uVar4 != 0) && ((*(uint *)((int)pvVar5 + 0x50) & 8) != 0)) &&
       ((*(uint *)((int)pvVar5 + 0x50) & 0x4000) == 0)) {
      local_50 = 0xf2;
      local_4c = 0xf3;
      local_4 = 0;
      uVar2 = (**(code **)(*local_54 + 4))();
      if ((uVar2 < local_50) || (local_4c <= uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        FUN_004fcd00(auStack_2c,this,4);
        local_4 = 1;
        FUN_00513120((int)auStack_2c);
        while (iStack_10 != 0) {
          pvVar5 = (void *)FUN_0052bed0((int)auStack_2c);
          FUN_005130d0((int)auStack_2c);
          uVar2 = FUN_004fcb50(pvVar5,param_1);
          if ((uVar2 == 0) || (uVar4 == 0)) {
            uVar4 = 0;
          }
          else {
            uVar4 = 1;
          }
        }
        FUN_004fd450(&uStack_48,(int)param_1);
        local_4._0_1_ = 2;
        uStack_44 = 1;
        uStack_40 = 3;
        if ((*(byte *)((int)this + 0x50) & 2) == 0) {
          FUN_004fd620(&uStack_48,*(uint *)((int)this + 0x24) >> 4 & 3,0);
        }
                    /* WARNING: Load size is inaccurate */
        iVar3 = (**(code **)(*this + 0xdc))(&uStack_48);
        if ((iVar3 == 0) || (uVar4 == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        iVar3 = FUN_004fa580(this,&uStack_48);
        if ((iVar3 == 0) || (!bVar1)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        uVar4 = FUN_004fca30(this,&uStack_48);
        if ((uVar4 == 0) || (!bVar1)) {
          uVar4 = 0;
        }
        else {
          uVar4 = 1;
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_004fd4d0(&uStack_48);
        local_4 = 0xffffffff;
        FUN_004fcd80(auStack_2c);
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

