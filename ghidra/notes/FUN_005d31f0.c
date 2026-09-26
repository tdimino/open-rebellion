
void __fastcall FUN_005d31f0(int *param_1)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  void *unaff_ESI;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065469b;
  local_c = ExceptionList;
  iVar2 = param_1[0xe];
  if (((iVar2 == 2) || (iVar2 == 3)) || (iVar2 == 4)) {
    ExceptionList = &local_c;
    (**(code **)(*param_1 + 0x30))();
    param_1[0xe] = 1;
  }
  else if (iVar2 == 0) {
    ExceptionList = &local_c;
    param_1[0xe] = 1;
    if (param_1[0x10] == 0) {
      iVar2 = (**(code **)(*(int *)param_1[0xb] + 0x48))((int *)param_1[0xb],param_1[0x13],0);
      FUN_005d8b00(iVar2);
      iVar2 = param_1[0x13];
    }
    else {
      piVar1 = (int *)param_1[0xb];
      if (param_1[0x10] == 1) {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,param_1[0x12],0);
        FUN_005d8b00(iVar2);
        iVar2 = param_1[0x12];
      }
      else {
        iVar2 = (**(code **)(*piVar1 + 0x48))(piVar1,param_1[0x11]);
        FUN_005d8b00(iVar2);
        iVar2 = param_1[0x11];
      }
    }
    param_1[0xd] = iVar2;
    pvVar3 = (void *)FUN_00618b70(0x14);
    local_c = (void *)0x0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_005ee4f0(pvVar3,(int)param_1);
    }
    local_c = (void *)0xffffffff;
    FUN_005f58d0((void *)(DAT_006bcbd4 + 0x5f4),(int)pvVar3);
    ExceptionList = unaff_ESI;
    return;
  }
  ExceptionList = local_c;
  return;
}

