
uint * __thiscall FUN_005f9550(void *this,char param_1)

{
  void *this_00;
  int iVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  uint uVar4;
  undefined3 extraout_var_00;
  undefined4 *puVar5;
  void *this_01;
  void *this_02;
  undefined3 extraout_var_01;
  uint *puVar6;
  void *local_c;
  undefined1 *puStack_8;
  void *local_4;
  
  local_4 = (void *)0xffffffff;
  puStack_8 = &LAB_00655c2b;
  local_c = ExceptionList;
  this_00 = (void *)((int)this + 0x78);
  puVar6 = (uint *)0x0;
  ExceptionList = &local_c;
  iVar3 = thunk_FUN_005f5060((int)this_00);
  while ((iVar1 = iVar3, iVar1 != 0 && (*(int *)((int)this + 0x108) != 0))) {
    iVar3 = *(int *)(iVar1 + 0x10);
    bVar2 = FUN_005fb470(iVar1);
    if (CONCAT31(extraout_var,bVar2) != 0) {
      uVar4 = FUN_005fb490(iVar1);
      if ((char)uVar4 == '\0') {
        puVar6 = FUN_005fb500(iVar1);
        if (puVar6 != (uint *)0x0) {
          FUN_005fb630(iVar1);
          FUN_005f54a0(this_00,*(uint *)(iVar1 + 0x18));
          bVar2 = FUN_005fa350(this,(int)puVar6);
          if (CONCAT31(extraout_var_00,bVar2) != 0) {
            if ((undefined *)puVar6[1] != (undefined *)0x0) {
              FUN_00618b60((undefined *)puVar6[1]);
            }
            puVar6[1] = 0;
            FUN_00618b60((undefined *)puVar6);
            puVar6 = (uint *)0x0;
          }
          if (*(int *)((int)this + 0x108) == 0) {
            ExceptionList = local_c;
            return (uint *)0x0;
          }
        }
      }
      else {
        uVar4 = FUN_005fb490(iVar1);
        if (((char)uVar4 == param_1) && (puVar6 = FUN_005fb500(iVar1), puVar6 != (uint *)0x0)) {
          FUN_005fb630(iVar1);
          FUN_005f54a0(this_00,*(uint *)(iVar1 + 0x18));
          ExceptionList = local_c;
          return puVar6;
        }
      }
    }
  }
  puVar5 = FUN_005f9860(this);
  do {
    if (puVar5 == (undefined4 *)0x0) {
      ExceptionList = local_c;
      return puVar6;
    }
    uVar4 = (puVar5[8] - (int)*(char *)((int)puVar5 + 0x27)) + 1;
    this_01 = (void *)FUN_005f5500(this_00,uVar4);
    if (this_01 == (void *)0x0) {
      this_02 = (void *)FUN_00618b70(0x28);
      if (this_02 == (void *)0x0) {
        this_01 = (void *)0x0;
      }
      else {
        local_4 = this_01;
        this_01 = FUN_005fb410(this_02,uVar4);
      }
      local_4 = (void *)0xffffffff;
      FUN_005f5440(this_00,this_01);
    }
    iVar3 = FUN_005fb4a0(this_01,puVar5);
    if (iVar3 == 0) {
LAB_005f9745:
      if (puVar6 != (uint *)0x0) {
        ExceptionList = local_c;
        return puVar6;
      }
    }
    else {
      uVar4 = FUN_005fb490((int)this_01);
      if ((char)uVar4 != '\0') {
        uVar4 = FUN_005fb490((int)this_01);
        if ((char)uVar4 == param_1) {
          puVar6 = FUN_005fb500((int)this_01);
          if (puVar6 == (uint *)0x0) goto LAB_005f9749;
          FUN_005fb630((int)this_01);
          FUN_005f54a0(this_00,*(uint *)((int)this_01 + 0x18));
        }
        goto LAB_005f9745;
      }
      puVar6 = FUN_005fb500((int)this_01);
      if (puVar6 != (uint *)0x0) {
        FUN_005fb630((int)this_01);
        FUN_005f54a0(this_00,*(uint *)((int)this_01 + 0x18));
        bVar2 = FUN_005fa350(this,(int)puVar6);
        if (CONCAT31(extraout_var_01,bVar2) != 0) {
          if ((undefined *)puVar6[1] != (undefined *)0x0) {
            FUN_00618b60((undefined *)puVar6[1]);
          }
          puVar6[1] = 0;
          FUN_00618b60((undefined *)puVar6);
          puVar6 = (uint *)0x0;
        }
        if (*(int *)((int)this + 0x108) == 0) {
          ExceptionList = local_c;
          return (uint *)0x0;
        }
        goto LAB_005f9745;
      }
    }
LAB_005f9749:
    puVar5 = FUN_005f9860(this);
  } while( true );
}

