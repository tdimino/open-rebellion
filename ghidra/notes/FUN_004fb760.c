
undefined4 __thiscall FUN_004fb760(void *this,undefined4 param_1,void *param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  
  iVar2 = FUN_004f72f0(this,0,param_2);
  iVar3 = (**(code **)(*(int *)this + 0x8c))(param_2);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = (**(code **)(*(int *)this + 0x90))(param_2);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_004f8240(this,param_2);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_004f81c0(this,param_2);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_004f80e0(this,param_2);
  if ((iVar2 != 0) && (bVar1)) {
    return 1;
  }
  return 0;
}

