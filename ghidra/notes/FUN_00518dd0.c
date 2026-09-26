
uint __thiscall FUN_00518dd0(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  
  uVar2 = FUN_004f9340(this,param_1);
  if ((uVar2 != 0) && ((*(uint *)((int)this + 0x24) & 0x30) == 0)) {
    iVar3 = FUN_005197c0((int)this);
    uVar2 = 0;
    if (iVar3 != 0) {
      bVar1 = FUN_00519740((int)this);
      uVar2 = 0;
      if (CONCAT31(extraout_var,bVar1) != 0) {
        bVar1 = FUN_005197f0((int)this);
        uVar2 = 0;
        if (CONCAT31(extraout_var_00,bVar1) != 0) {
          iVar3 = (**(code **)(*(int *)((int)this + 0x58) + 4))(param_1);
          uVar2 = 0;
          if (iVar3 != 0) {
            FUN_005f4d90(param_1,(int)this + 0x6c);
            FUN_005f4d90(param_1,(int)this + 0x70);
            FUN_005f4d90(param_1,(int)this + 0x74);
            FUN_005f4d90(param_1,(int)this + 0x78);
            FUN_005f4d90(param_1,(int)this + 0x7c);
            FUN_005f4d90(param_1,(int)this + 0x80);
            FUN_005f4d90(param_1,(int)this + 0x84);
            FUN_005f4d90(param_1,(int)this + 0x88);
            iVar3 = FUN_004ece90((void *)((int)this + 0x8c),param_1);
            uVar2 = 0;
            if (iVar3 != 0) {
              uVar2 = FUN_004fd4e0((void *)((int)this + 0x90),param_1);
              if (uVar2 != 0) {
                FUN_005f57f0(*(void **)((int)this + 0xac),param_1);
              }
            }
          }
        }
      }
    }
  }
  return uVar2;
}

