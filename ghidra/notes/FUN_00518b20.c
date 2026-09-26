
int __cdecl FUN_00518b20(undefined4 param_1,uint *param_2,void *param_3)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  uint *puVar4;
  void *pvVar5;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642b58;
  local_c = ExceptionList;
  iVar3 = 1;
  if (param_2[6] != 0) {
    ExceptionList = &local_c;
    FUN_00512ef0(local_2c,param_1);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      puVar4 = param_2;
      pvVar5 = param_3;
      piVar1 = (int *)FUN_0052bed0((int)local_2c);
      uVar2 = FUN_00518be0(piVar1,puVar4,pvVar5);
      if ((uVar2 == 0) || (iVar3 == 0)) {
        iVar3 = 0;
      }
      else {
        iVar3 = 1;
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_00513010(local_2c);
  }
  ExceptionList = local_c;
  return iVar3;
}

