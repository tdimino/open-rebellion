
uint __thiscall FUN_00551c80(void *this,uint *param_1,uint *param_2,void *param_3,void *param_4)

{
  uint uVar1;
  bool bVar2;
  int *local_fc;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0 [57];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648cd6;
  local_c = ExceptionList;
  uVar1 = 1;
  local_f8 = 1;
  local_f4 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_f8);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0054fb20((int)param_4);
  local_fc = (int *)0x0;
  FUN_00550100(local_f0);
  local_4 = 1;
  FUN_005501c0((int)local_f0);
  if (*(int *)((int)param_3 + 4) == -1) {
    uVar1 = FUN_00552f40(this,param_1,param_3,(int *)&local_fc);
  }
  bVar2 = uVar1 == 0;
  if (!bVar2) {
    if (*(int *)((int)param_3 + 4) == -1) {
      uVar1 = FUN_00550200(local_f0,local_fc,(uint *)((int)this + 0x20),1,param_3);
    }
    bVar2 = uVar1 == 0;
  }
  if ((!bVar2) && (*(int *)((int)param_3 + 4) == -1)) {
    uVar1 = FUN_00552860(this,(int)local_f0,1,param_2,param_3,param_4);
  }
  if ((uVar1 != 0) && (*(int *)((int)param_3 + 4) == -1)) {
    uVar1 = FUN_00552dd0(this,(int)local_f0,1,param_2);
  }
  local_4 = 0xffffffff;
  FUN_00550170(local_f0);
  ExceptionList = local_c;
  return uVar1;
}

