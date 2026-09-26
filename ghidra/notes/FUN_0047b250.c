
int * __fastcall FUN_0047b250(int *param_1)

{
  uint uVar1;
  bool bVar2;
  int *piVar3;
  undefined3 extraout_var;
  int iVar4;
  void *pvVar5;
  int *piVar6;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006336c3;
  pvStack_c = ExceptionList;
  piVar6 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  piVar3 = FUN_004f3000(param_1[9],(uint *)(param_1 + 10));
  if (piVar3 != (int *)0x0) {
    bVar2 = FUN_00520b30((int)piVar3);
    if (CONCAT31(extraout_var,bVar2) != 0) {
      if ((param_1[8] & 0x40000000U) != 0) {
        iVar4 = (**(code **)(*param_1 + 0x34))();
        if (iVar4 == 0) {
          uVar1 = param_1[8];
          param_1[8] = uVar1 & 0xbfffffff;
          param_1[8] = uVar1 & 0xbfffffff | 0x20000000;
        }
      }
      if ((param_1[8] & 0x20000000U) != 0) {
        piVar6 = (int *)FUN_004f5cd0(0x250);
        if (piVar6 != (int *)0x0) {
          piVar6[8] = param_1[9];
          pvVar5 = (void *)FUN_00618b70(0x20);
          local_4._0_1_ = 1;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            pvVar5 = FUN_004f5b10(pvVar5,param_1 + 10,0);
          }
          local_4 = (uint)local_4._1_3_ << 8;
          if (pvVar5 != (void *)0x0) {
            FUN_004f57b0(local_14,pvVar5);
          }
          (**(code **)(*piVar6 + 0x24))(local_14);
        }
      }
      param_1[8] = param_1[8] & 0x9fffffff;
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar6;
}

