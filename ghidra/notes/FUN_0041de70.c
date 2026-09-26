
uint __thiscall FUN_0041de70(void *this,int *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  void *this_00;
  uint uVar2;
  int *piVar3;
  
  FUN_005f4d70(param_1);
  FUN_005f4db0(param_1,(int)this + 4);
  FUN_005f4db0(param_1,(int)this + 8);
  FUN_005f4db0(param_1,(int)this + 0xc);
  FUN_005f4db0(param_1,(int)this + 0x10);
  FUN_005f4db0(param_1,(int)this + 0x14);
  FUN_005f4db0(param_1,(int)this + 0x18);
  FUN_005f4db0(param_1,(int)this + 0x1c);
  FUN_005f4d70(param_1);
  bVar1 = FUN_0041e1a0();
  uVar2 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    piVar3 = param_1;
    this_00 = (void *)FUN_0051ce00();
    uVar2 = FUN_0051d2c0(this_00,piVar3);
    if (uVar2 != 0) {
      FUN_005f4d70(param_1);
      if (uVar2 != 0) {
        uVar2 = FUN_0041cd50(param_1);
        if (uVar2 != 0) {
          FUN_005f4d70(param_1);
        }
      }
    }
  }
  return uVar2;
}

