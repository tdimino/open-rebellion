
uint __thiscall FUN_005513a0(void *this,uint *param_1)

{
  bool bVar1;
  uint *puVar2;
  void *pvVar3;
  uint uVar4;
  undefined3 extraout_var;
  int *piVar5;
  undefined4 uVar6;
  uint uVar7;
  undefined4 uVar8;
  int *local_58;
  int *local_54;
  uint local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648c10;
  local_c = ExceptionList;
  uVar7 = 1;
  local_58 = (int *)0x0;
  ExceptionList = &local_c;
  if (*(int *)((int)this + 0x10) == -1) {
    ExceptionList = &local_c;
    uVar7 = FUN_00552e80(this,param_1,(void *)((int)this + 0xc),(int *)&local_58);
  }
  param_1 = (uint *)0x0;
  if (uVar7 != 0) {
    if (*(int *)((int)this + 0x10) == -1) {
      uVar7 = FUN_00552ff0(this,local_58,(void *)((int)this + 0xc),(int *)&param_1);
    }
    if (uVar7 != 0) {
      if (*(int *)((int)this + 0x10) == -1) {
        uVar7 = FUN_005531b0(local_58,(void *)((int)this + 0xc));
      }
      if ((uVar7 != 0) && (*(int *)((int)this + 0x10) == -1)) {
        puVar2 = FUN_004025b0(param_1,(uint *)&local_54);
        local_4 = 0;
        pvVar3 = FUN_004f5940(this,puVar2);
        local_4 = 0xffffffff;
        FUN_00619730();
        if (pvVar3 == (void *)0x0) {
          uVar4 = FUN_00552d10(this,(int *)param_1);
          uVar7 = 0;
          if (uVar4 != 0) {
            local_54 = (int *)0x0;
            bVar1 = FUN_004f6b50(param_1,(int *)&local_54);
            uVar7 = CONCAT31(extraout_var,bVar1);
            if (uVar7 != 0) {
              puVar2 = FUN_004025b0(local_54,&local_50);
              local_4 = 1;
              pvVar3 = FUN_004f5940(this,puVar2);
              local_4 = 0xffffffff;
              FUN_00619730();
              if (pvVar3 == (void *)0x0) {
                uVar7 = FUN_00552d10(this,local_54);
              }
            }
          }
          FUN_004ffe70(local_4c,param_1,4);
          local_4 = 2;
          FUN_00513120((int)local_4c);
          while ((uVar7 != 0 && (local_30 != 0))) {
            piVar5 = (int *)FUN_0052bed0((int)local_4c);
            uVar7 = FUN_00552d10(this,piVar5);
            if (uVar7 != 0) {
              uVar8 = 4;
              uVar6 = FUN_0052bed0((int)local_4c);
              FUN_00502db0(local_2c,uVar6,uVar8);
              local_4 = CONCAT31(local_4._1_3_,3);
              FUN_00513120((int)local_2c);
              do {
                if (local_10 == 0) break;
                piVar5 = (int *)FUN_0052bed0((int)local_2c);
                uVar7 = FUN_00552d10(this,piVar5);
                FUN_005130d0((int)local_2c);
              } while (uVar7 != 0);
              local_4 = CONCAT31(local_4._1_3_,2);
              FUN_00502eb0(local_2c);
            }
            FUN_005130d0((int)local_4c);
          }
          local_4 = 0xffffffff;
          FUN_004fff70(local_4c);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar7;
}

