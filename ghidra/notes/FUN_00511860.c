
int __thiscall FUN_00511860(void *this,int param_1,int param_2)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  uint uVar4;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642118;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,param_2);
  local_4 = 0;
  local_24 = 1;
  local_20 = 3;
  pvVar1 = (void *)FUN_00539fd0(this,1);
  iVar2 = FUN_00510580(pvVar1,&local_28);
  pvVar1 = (void *)FUN_00539fd0(this,2);
  iVar3 = FUN_00510580(pvVar1,&local_28);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    iVar2 = 0;
  }
  else {
    iVar2 = 1;
  }
  if (param_1 != 0) {
    uVar4 = FUN_0050d510(this,param_2);
    if ((uVar4 == 0) || (iVar2 == 0)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return iVar2;
}

