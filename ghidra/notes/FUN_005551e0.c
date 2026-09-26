
uint __thiscall FUN_005551e0(void *this,int *param_1,uint *param_2,uint *param_3,int *param_4)

{
  void *pvVar1;
  uint uVar2;
  int *piVar3;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  int *local_14;
  int *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649400;
  local_c = ExceptionList;
  local_20 = 1;
  local_1c = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_20);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0058af30(&local_18);
  local_4 = 1;
  uVar2 = FUN_00554150(this,param_2,(int)&local_18);
  piVar3 = local_10;
  if (local_14 != (int *)0x0) {
    piVar3 = local_14;
    local_10 = local_14;
  }
  while (((uVar2 != 0 && (pvVar1 = (void *)*piVar3, pvVar1 != (void *)0x0)) && (param_4[1] != -1)))
  {
    if (((byte)*(undefined4 *)((int)pvVar1 + 0x24) & 0xc0) == 0xc0) {
      uVar2 = FUN_005571c0(param_1,pvVar1,0,param_3,param_4);
      piVar3 = local_10;
    }
    if (*piVar3 != 0) {
      piVar3 = piVar3 + 1;
      local_10 = piVar3;
    }
  }
  local_4 = 0xffffffff;
  FUN_0058af70(&local_18);
  ExceptionList = local_c;
  return uVar2;
}

