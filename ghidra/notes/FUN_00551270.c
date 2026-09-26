
uint __thiscall FUN_00551270(void *this,uint *param_1)

{
  bool bVar1;
  void *pvVar2;
  uint uVar3;
  undefined3 extraout_var;
  uint *puVar4;
  uint uVar5;
  int *local_18;
  int local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648bd8;
  local_c = ExceptionList;
  uVar5 = 1;
  local_18 = (int *)0x0;
  ExceptionList = &local_c;
  if (*(int *)((int)this + 0x10) == -1) {
    ExceptionList = &local_c;
    uVar5 = FUN_00552e80(this,param_1,(void *)((int)this + 0xc),(int *)&local_18);
  }
  local_14 = 0;
  if (uVar5 != 0) {
    if (*(int *)((int)this + 0x10) == -1) {
      uVar5 = FUN_00552ff0(this,local_18,(void *)((int)this + 0xc),&local_14);
    }
    if (uVar5 != 0) {
      if (*(int *)((int)this + 0x10) == -1) {
        uVar5 = FUN_005531b0(local_18,(void *)((int)this + 0xc));
      }
      if ((uVar5 != 0) && (*(int *)((int)this + 0x10) == -1)) {
        pvVar2 = FUN_004f5940(this,puVar4);
        if (pvVar2 == (void *)0x0) {
          uVar3 = FUN_00552d10(this,local_18);
          uVar5 = 0;
          if (uVar3 != 0) {
            param_1 = (uint *)0x0;
            bVar1 = FUN_004f6b50(local_18,(int *)&param_1);
            uVar5 = CONCAT31(extraout_var,bVar1);
            if (uVar5 != 0) {
              puVar4 = FUN_004025b0(param_1,&local_10);
              local_4 = 0;
              pvVar2 = FUN_004f5940(this,puVar4);
              local_4 = 0xffffffff;
              FUN_00619730();
              if (pvVar2 == (void *)0x0) {
                uVar5 = FUN_00552d10(this,(int *)param_1);
              }
            }
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar5;
}

