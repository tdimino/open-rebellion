
uint __thiscall FUN_0041dd60(void *this,int *param_1)

{
  int iVar1;
  uint uVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  void *this_00;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  bool bVar3;
  int *piVar4;
  
  FUN_0041e550();
  FUN_004fcde0();
  iVar1 = FUN_0041e360((int)this);
  bVar3 = iVar1 == 0;
  uVar2 = 0;
  if (!bVar3) {
    uVar2 = FUN_0041e330((int)this);
    bVar3 = uVar2 == 0;
  }
  if (!bVar3) {
    bVar3 = FUN_005f4d40(param_1);
    uVar2 = CONCAT31(extraout_var,bVar3);
  }
  if (uVar2 != 0) {
    FUN_005f4d90(param_1,(int)this + 4);
    FUN_005f4d90(param_1,(int)this + 8);
    FUN_005f4d90(param_1,(int)this + 0xc);
    FUN_005f4d90(param_1,(int)this + 0x10);
    FUN_005f4d90(param_1,(int)this + 0x14);
    FUN_005f4d90(param_1,(int)this + 0x18);
    FUN_005f4d90(param_1,(int)this + 0x1c);
    if (uVar2 != 0) {
      bVar3 = FUN_005f4d40(param_1);
      uVar2 = 0;
      if (CONCAT31(extraout_var_00,bVar3) != 0) {
        FUN_0051ce10();
        bVar3 = FUN_0041e1a0();
        uVar2 = 0;
        if (CONCAT31(extraout_var_01,bVar3) != 0) {
          piVar4 = param_1;
          this_00 = (void *)FUN_0051ce00();
          iVar1 = FUN_0051cec0(this_00,piVar4);
          uVar2 = 0;
          if (iVar1 != 0) {
            bVar3 = FUN_005f4d40(param_1);
            uVar2 = 0;
            if (CONCAT31(extraout_var_02,bVar3) != 0) {
              iVar1 = FUN_0041cd10(param_1);
              uVar2 = 0;
              if (iVar1 != 0) {
                bVar3 = FUN_005f4d40(param_1);
                uVar2 = CONCAT31(extraout_var_03,bVar3);
              }
            }
          }
        }
      }
    }
  }
  return uVar2;
}

