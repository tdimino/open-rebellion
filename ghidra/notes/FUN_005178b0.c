
int __cdecl FUN_005178b0(void *param_1)

{
  bool bVar1;
  void *this;
  int *this_00;
  uint uVar2;
  uint *puVar3;
  uint *puVar4;
  void *this_01;
  int iVar5;
  int iVar6;
  void *pvVar7;
  void *pvVar8;
  void *local_4c;
  void *local_48;
  uint local_44;
  uint local_40;
  uint local_3c;
  undefined4 local_38;
  uint uStack_34;
  undefined4 uStack_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642980;
  local_c = ExceptionList;
  iVar6 = 1;
  ExceptionList = &local_c;
  this = (void *)FUN_005069d0();
  while (this != (void *)0x0) {
    local_4c = (void *)0x0;
    this_00 = (int *)FUN_005061f0();
    while ((this_00 != (int *)0x0 && (local_4c == (void *)0x0))) {
      if ((*(byte *)(this_00 + 0x14) & 1) != 0) {
        local_48 = local_4c;
        uVar2 = FUN_00518690(this_00,(int *)&local_48);
        if ((uVar2 == 0) || (iVar6 == 0)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        if ((local_48 == (void *)0x0) || (!bVar1)) {
          iVar6 = 0;
        }
        else {
          iVar6 = 1;
        }
        if (local_48 != (void *)0x0) {
          puVar3 = FUN_004025b0(local_48,&local_40);
          local_4 = 2;
          puVar4 = FUN_004025b0(this,&local_44);
          local_4c = (void *)(uint)(*puVar4 == *puVar3);
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
      local_3c = 0x18;
      local_38 = 0x1c;
      local_4 = 1;
      this_00 = (int *)FUN_004f6010(this_00,&local_3c,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    FUN_00512f90(local_2c,this,1);
    local_4 = 3;
    FUN_00513120((int)local_2c);
    while (local_10 != 0) {
      pvVar7 = local_4c;
      pvVar8 = param_1;
      this_01 = (void *)FUN_0052bed0((int)local_2c);
      iVar5 = FUN_0050a890(this_01,(int)pvVar7,pvVar8);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
      FUN_005130d0((int)local_2c);
    }
    local_4 = 0xffffffff;
    FUN_00513010(local_2c);
    uStack_34 = 0x80;
    uStack_30 = 0x90;
    local_4 = 0;
    this = (void *)FUN_004f6010(this,&uStack_34,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar6;
}

