
void __thiscall FUN_00419110(void *this,undefined4 *param_1)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  uint uVar5;
  uint local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b180;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(local_20);
  bVar2 = false;
  local_4 = 0;
  FUN_004f26d0(local_20,param_1);
  do {
    local_20[1] = 0x90;
    uVar5 = local_20[0] >> 0x18;
    local_20[2] = 0x98;
    if ((uVar5 < 0x90) || (0x97 < uVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_004311a0((void *)((int)this + 0x2c),local_20);
LAB_00419195:
      bVar2 = true;
    }
    else {
      local_20[3] = 8;
      local_20[4] = 0x10;
      if ((uVar5 < 8) || (0xf < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_0042ffe0((void *)((int)this + 0x78),local_20);
        goto LAB_00419195;
      }
      piVar3 = FUN_004f2d10(*(int *)this,local_20);
      if ((piVar3 == (int *)0x0) || ((void *)piVar3[7] == (void *)0x0)) goto LAB_00419195;
      puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
      local_4._0_1_ = 1;
      FUN_004f26d0(local_20,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    if (bVar2) {
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
  } while( true );
}

