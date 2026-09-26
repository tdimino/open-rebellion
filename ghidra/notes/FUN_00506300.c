
int __cdecl FUN_00506300(uint *param_1,undefined4 *param_2,uint *param_3,void *param_4)

{
  int *piVar1;
  uint uVar2;
  bool bVar3;
  int iVar4;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00641287;
  pvStack_c = ExceptionList;
  bVar3 = false;
  iVar4 = 1;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar1 = (int *)FUN_0053efa0(param_1);
  if (piVar1 != (int *)0x0) {
    local_14 = 0x18;
    local_10 = 0x1c;
    local_4 = 1;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < local_14) || (local_10 <= uVar2)) {
      iVar4 = 0;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 != 0) goto LAB_005063aa;
    piVar1 = (int *)0x0;
  }
  if (iVar4 == 0) {
    ExceptionList = pvStack_c;
    return 0;
  }
LAB_005063aa:
  *param_2 = piVar1;
  if ((piVar1 != (int *)0x0) && ((*(byte *)(piVar1 + 0x14) & 8) == 0)) {
    bVar3 = true;
    local_10 = 0xffffffff;
    local_14 = 1;
    param_3 = &local_14;
    local_4 = 2;
  }
  FUN_00520580(param_4,param_3);
  local_4 = 0xffffffff;
  if (bVar3) {
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return iVar4;
}

