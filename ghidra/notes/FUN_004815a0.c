
void __fastcall FUN_004815a0(void *param_1)

{
  int iVar1;
  void *this;
  bool bVar2;
  short sVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  uint local_1c;
  undefined1 local_18 [4];
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006341a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  iVar1 = (int)param_1 + 0x3c;
  local_4 = 0;
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xfffffffe;
  sVar3 = FUN_005f50e0(iVar1);
  if (sVar3 == 0) {
    pvVar4 = FUN_004035d0(*(void **)((int)param_1 + 100),0,2,1,0x7404051,0,0,0x45,10000,(void *)0x1)
    ;
    this = (void *)((int)param_1 + 0x54);
    FUN_00435790(this,(int)pvVar4);
    puVar5 = (undefined4 *)FUN_004357b0(this,local_18);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_1c,puVar5);
    do {
      local_4._0_1_ = 0;
      FUN_00619730();
      local_14 = 0x30;
      local_10 = 0x40;
      local_4 = CONCAT31(local_4._1_3_,2);
      if ((local_1c >> 0x18 < 0x30) || (0x3f < local_1c >> 0x18)) {
LAB_0048167a:
        bVar2 = false;
      }
      else {
        sVar3 = FUN_005f50e0(iVar1);
        if (sVar3 != 0) goto LAB_0048167a;
        bVar2 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (!bVar2) goto LAB_004816d3;
      iVar6 = FUN_0047ba90(param_1,&local_1c);
      if (iVar6 != 0) {
        FUN_0047b610(param_1,&local_1c);
      }
      puVar5 = (undefined4 *)FUN_004357b0(this,local_18);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_1c,puVar5);
    } while( true );
  }
LAB_004816da:
  sVar3 = FUN_005f50e0(iVar1);
  if (sVar3 != 0) {
    *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 1;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
LAB_004816d3:
  FUN_005f58b0(this);
  goto LAB_004816da;
}

