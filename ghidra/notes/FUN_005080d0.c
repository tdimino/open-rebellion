
undefined4 __thiscall FUN_005080d0(void *this,uint param_1,void *param_2)

{
  void *pvVar1;
  int iVar2;
  undefined4 uVar3;
  bool bVar4;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641798;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,(int)param_2);
  local_4 = 0;
  local_24 = 1;
  local_20 = 3;
  pvVar1 = (void *)FUN_00539fd0(this,1);
  if (pvVar1 != (void *)0x0) {
    iVar2 = FUN_0050ff50(pvVar1,&local_28);
    bVar4 = iVar2 != 0;
  }
  pvVar1 = (void *)FUN_00539fd0(this,2);
  if (pvVar1 != (void *)0x0) {
    iVar2 = FUN_0050ff50(pvVar1,&local_28);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  iVar2 = FUN_004f8660(this,param_1,param_2);
  if ((iVar2 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  iVar2 = FUN_004f9510(this,1,&local_28);
  if ((iVar2 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  iVar2 = FUN_004f9510(this,2,&local_28);
  if ((iVar2 == 0) || (!bVar4)) {
    uVar3 = 0;
  }
  else {
    uVar3 = 1;
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return uVar3;
}

