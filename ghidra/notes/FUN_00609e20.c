
void __fastcall FUN_00609e20(void *param_1)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  int *piVar5;
  undefined4 local_4;
  
  local_4 = 0;
  puVar4 = (undefined4 *)FUN_00609dd0(param_1,&local_4,0);
  local_4 = *puVar4;
  piVar5 = (int *)(**(code **)(**(int **)((int)param_1 + 0xa0) + 8))();
  uVar3 = local_4;
  while (piVar5 != (int *)0x0) {
    local_4._0_2_ = (short)uVar3;
    iVar1 = *(int *)((int)param_1 + 0xcc);
    local_4._2_2_ = (short)((uint)uVar3 >> 0x10);
    iVar2 = *(int *)((int)param_1 + 0xd0);
    piVar5[0x10] = piVar5[10] + (int)(short)local_4;
    piVar5[0x11] = piVar5[0xb] + (int)local_4._2_2_;
    piVar5[0x12] = piVar5[10] + (int)(short)local_4 + iVar1;
    piVar5[0x13] = piVar5[0xb] + (int)local_4._2_2_ + iVar2;
    local_4 = uVar3;
    piVar5 = (int *)(**(code **)(*piVar5 + 0xc))();
    uVar3 = local_4;
  }
  return;
}

