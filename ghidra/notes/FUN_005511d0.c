
void __thiscall FUN_005511d0(void *this,int *param_1,undefined4 param_2,undefined4 *param_3)

{
  byte bVar1;
  uint uVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648bb8;
  pvStack_c = ExceptionList;
  bVar1 = 0;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((0x2f < uVar2) && (uVar2 < 0x40)) {
    bVar1 = 1;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00551100(this,(uint)param_1[9] >> 6 & 3,param_2,(-(uint)bVar1 & 0x3d) + 0x204,param_3);
  ExceptionList = pvStack_c;
  return;
}

