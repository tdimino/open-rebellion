
undefined4 __fastcall FUN_0055abf0(int *param_1)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  void *unaff_ESI;
  undefined4 local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649b08;
  pvStack_c = ExceptionList;
  iVar2 = param_1[0x1b];
  local_18 = 0;
  ExceptionList = &pvStack_c;
  pvVar1 = FUN_0052e740(param_1,local_14);
  uVar3 = (uint)~param_1[0x18] >> 4 & 1;
  local_4 = 0;
  iVar2 = (**(code **)(*param_1 + 0x218))(pvVar1,iVar2,uVar3,&local_18);
  local_14[0] = 0xffffffff;
  FUN_00619730();
  if (iVar2 == 0) {
    uVar3 = 0;
  }
  FUN_00501ba0(param_1,uVar3);
  ExceptionList = unaff_ESI;
  return 1;
}

