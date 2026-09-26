
void * __cdecl FUN_0045f970(undefined2 param_1,undefined4 param_2)

{
  char *pcVar1;
  void *pvVar2;
  LPCSTR pCVar3;
  undefined2 uStack00000006;
  byte bVar4;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630f63;
  local_c = ExceptionList;
  uStack00000006 = 5;
  ExceptionList = &local_c;
  FUN_005f2fc0(local_18,&param_1);
  local_4 = 0;
  pcVar1 = (char *)FUN_0045f7b0();
  FUN_005f35b0(local_24,pcVar1);
  local_4._0_1_ = 1;
  FUN_00618b60(pcVar1);
  FUN_005f3110(local_24,local_30,(int)local_18);
  local_4._0_1_ = 2;
  pvVar2 = (void *)FUN_00618b70(0x28);
  local_4._0_1_ = 3;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    bVar4 = 9;
    pCVar3 = (LPCSTR)FUN_00583c40((int)local_30);
    pvVar2 = FUN_005fbc80(pvVar2,param_2,pCVar3,bVar4);
  }
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_30);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_24);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return pvVar2;
}

