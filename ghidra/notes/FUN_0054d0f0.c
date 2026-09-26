
undefined4 __thiscall FUN_0054d0f0(void *this,void *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_0054f4b0(param_1);
  FUN_005f60f0(param_1);
  FUN_005f60f0(param_1);
  FUN_005f60f0(param_1);
  iVar3 = FUN_005589b0(param_1);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_005589b0(param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = (**(code **)(*(int *)((int)this + 100) + 0x20))(param_1);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = (**(code **)(*(int *)((int)this + 0x6c) + 0x20))(param_1);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

