
void * FUN_00458fe0(void *param_1)

{
  bool bVar1;
  void *this;
  uint *puVar2;
  undefined4 uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  void *local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063067b;
  local_c = ExceptionList;
  bVar1 = false;
  uVar5 = 0;
  local_18 = (void *)0x0;
  uVar6 = uVar5;
  ExceptionList = &local_c;
  if (param_1 != (void *)0x0) {
    ExceptionList = &local_c;
    puVar2 = FUN_00402d80(param_1,&param_1);
    local_4 = 0;
    if ((0x27 < *puVar2 >> 0x18) && (*puVar2 >> 0x18 < 0x2c)) {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      uVar3 = FUN_00539f50((int)this);
      switch(uVar3) {
      case 1:
        uVar5 = 0x232e;
        break;
      case 2:
        uVar5 = 0x2336;
        break;
      case 3:
        uVar5 = 0x233e;
        break;
      case 4:
        uVar5 = 0x2332;
        break;
      case 5:
        uVar5 = 0x233a;
        break;
      case 6:
        uVar5 = 0x2342;
      }
    }
    else {
      puVar2 = FUN_00402d80(this,&param_1);
      bVar1 = true;
      local_4 = 1;
      if ((*puVar2 >> 0x18 < 0x2c) || (0x2f < *puVar2 >> 0x18)) {
        bVar1 = false;
      }
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        iVar4 = FUN_00539f50((int)this);
        if (iVar4 == 1) {
          uVar5 = 0x2329;
        }
        else if (iVar4 == 2) {
          uVar5 = 0x2346;
        }
      }
    }
    if ((*(uint *)((int)this + 0x50) & 4) == 0) {
      uVar6 = uVar5 + 1;
      if (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0x40) {
        uVar6 = uVar5 + 2;
      }
    }
    else {
      uVar6 = uVar5;
      if ((*(uint *)((int)this + 0x50) & 0x10) != 0) {
        uVar6 = uVar5 + 3;
      }
    }
  }
  if ((short)uVar6 != 0) {
    uVar3 = FUN_006037f0(7);
    param_1 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (param_1 == (void *)0x0) {
      local_18 = (void *)0x0;
    }
    else {
      local_18 = FUN_005fbd20(param_1,uVar3,uVar6,2);
    }
  }
  ExceptionList = local_c;
  return local_18;
}

