
void __thiscall FUN_005d1ed0(void *this,void *param_1)

{
  void *this_00;
  int iVar1;
  int iVar2;
  uint uVar3;
  uint local_8;
  int iStack_4;
  
  local_8 = 0;
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 4))();
  FUN_005f4d90(param_1,&iStack_4);
  iVar2 = 0;
  if (0 < iStack_4) {
    do {
      FUN_005f4d90(param_1,&local_8);
      uVar3 = local_8;
      this_00 = (void *)FUN_00596bd0();
      iVar1 = FUN_005a6340(this_00,uVar3);
      if (iVar1 != 0) {
        FUN_005d1ff0(this,iVar1);
      }
      iVar2 = iVar2 + 1;
    } while (iVar2 < iStack_4);
  }
  return;
}

