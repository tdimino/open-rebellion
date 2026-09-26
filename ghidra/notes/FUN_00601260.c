
void __fastcall FUN_00601260(int param_1)

{
  void *this;
  int iVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006560cb;
  local_c = ExceptionList;
  if (((DAT_006be4e4 != 0) && (*(int *)(DAT_006be4e4 + 0xc) != 0)) &&
     (iVar1 = *(int *)(*(int *)(DAT_006be4e4 + 0xc) + 0x1c), iVar1 != 0)) {
    this = (void *)(iVar1 + 0x88);
    ExceptionList = &local_c;
    iVar1 = FUN_0060f070(this,*(uint *)(param_1 + 0x18));
    if (iVar1 == 0) {
      pvVar2 = (void *)FUN_00618b70(0x20);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_0060f180(pvVar2,*(undefined4 *)(param_1 + 0x18));
      }
      local_4 = 0xffffffff;
      FUN_005f4f10(this,(int)pvVar2);
    }
  }
  ExceptionList = local_c;
  return;
}

