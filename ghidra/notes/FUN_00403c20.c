
void * __thiscall FUN_00403c20(void *this,void *param_1,undefined4 *param_2)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  uint local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006295e7;
  local_c = ExceptionList;
  bVar2 = false;
  local_1c[1] = 0;
  ExceptionList = &local_c;
  FUN_004ece30(local_1c);
  local_4 = 1;
  FUN_004f26d0(local_1c,param_2);
  do {
    local_1c[2] = 0x90;
    local_1c[3] = 0x98;
    if ((local_1c[0] >> 0x18 < 0x90) || (0x97 < local_1c[0] >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
LAB_00403ce0:
      bVar2 = true;
    }
    else {
      piVar3 = FUN_004f2d10(*(int *)((int)this + 8),local_1c);
      if ((piVar3 == (int *)0x0) || ((void *)piVar3[7] == (void *)0x0)) goto LAB_00403ce0;
      puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_2);
      local_4._0_1_ = 2;
      FUN_004f26d0(local_1c,puVar4);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
    if (bVar2) {
      FUN_004f26d0(param_1,local_1c);
      local_1c[1] = 1;
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      ExceptionList = local_c;
      return param_1;
    }
  } while( true );
}

