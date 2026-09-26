
int __thiscall FUN_004f8570(void *this,undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  void *unaff_EBX;
  int unaff_ESI;
  undefined4 unaff_retaddr;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063fe18;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x68))(param_1,param_2,&local_14);
  if ((iVar1 != 0) && (unaff_ESI == -1)) {
    iVar1 = (**(code **)(*(int *)this + 0xa4))(param_2,unaff_retaddr);
  }
  local_10 = 0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EBX;
  return iVar1;
}

