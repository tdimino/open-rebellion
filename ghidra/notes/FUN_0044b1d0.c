
void FUN_0044b1d0(undefined4 param_1,int param_2,void *param_3)

{
  void *this;
  char *pcVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f42b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00618b70(0x68);
  pvVar2 = (void *)0x0;
  local_4 = 0;
  if (this != (void *)0x0) {
    pvVar2 = FUN_0060ba00(this,param_1);
  }
  local_4 = 0xffffffff;
  pcVar1 = (char *)FUN_00583c40(param_2);
  FUN_005f35e0((void *)((int)pvVar2 + 0x14),pcVar1);
  FUN_005f59f0(param_3,(int)pvVar2);
  ExceptionList = local_c;
  return;
}

