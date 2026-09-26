
uint __thiscall FUN_00552000(void *this,uint *param_1,void *param_2)

{
  void *pvVar1;
  uint uVar2;
  uint local_fc [2];
  int *local_f4;
  undefined4 local_f0 [57];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648d41;
  local_c = ExceptionList;
  uVar2 = 1;
  local_fc[0] = 1;
  local_fc[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,local_fc);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_f4 = (int *)0x0;
  FUN_00550100(local_f0);
  local_4 = 1;
  if (*(int *)((int)param_2 + 4) == -1) {
    uVar2 = FUN_00552f40(this,param_1,param_2,(int *)&local_f4);
  }
  if (uVar2 != 0) {
    if (*(int *)((int)param_2 + 4) == -1) {
      uVar2 = FUN_00550200(local_f0,local_f4,(uint *)((int)this + 0x20),0,param_2);
    }
    if ((uVar2 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
      FUN_004ece30(local_fc);
      local_4 = CONCAT31(local_4._1_3_,2);
      uVar2 = FUN_004f6b70(local_f4,local_fc);
      if (uVar2 != 0) {
        pvVar1 = FUN_004f5940(this,local_fc);
        if (pvVar1 != (void *)0x0) {
          uVar2 = FUN_00552dd0(this,(int)local_f0,0,local_fc);
        }
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
  }
  local_4 = 0xffffffff;
  FUN_00550170(local_f0);
  ExceptionList = local_c;
  return uVar2;
}

