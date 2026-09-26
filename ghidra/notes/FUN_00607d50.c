
uint * __thiscall FUN_00607d50(void *this,uint param_1)

{
  bool bVar1;
  int iVar2;
  void *pvVar3;
  uint *this_00;
  undefined3 extraout_var;
  HDC hDC;
  undefined4 *puVar4;
  uint uVar5;
  undefined3 extraout_var_00;
  uint *puVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006566b6;
  local_c = ExceptionList;
  puVar6 = (uint *)0x0;
  ExceptionList = &local_c;
  iVar2 = FUN_006037f0(*(int *)((int)this + 0x98));
  if (iVar2 != 0) {
    pvVar3 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar3 == (void *)0x0) {
      this_00 = (uint *)0x0;
    }
    else {
      this_00 = FUN_005fbd20(pvVar3,iVar2,param_1,2);
    }
    local_4 = 0xffffffff;
    bVar1 = FUN_005fc040((int *)this_00);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      hDC = GetDC(*(HWND *)((int)this + 0x18));
      puVar4 = (undefined4 *)FUN_005fc0e0((int *)this_00);
      uVar5 = FUN_005fc0f0((int *)this_00);
      pvVar3 = (void *)FUN_00618b70(0x28);
      local_4 = 1;
      if (pvVar3 == (void *)0x0) {
        puVar6 = (uint *)0x0;
      }
      else {
        puVar6 = FUN_005fbea0(pvVar3,(LONG)puVar4,uVar5,this_00,hDC);
      }
      local_4 = 0xffffffff;
      bVar1 = FUN_005fc040((int *)puVar6);
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        FUN_005fcc30(this_00,puVar6,0,0,0,0,puVar4,uVar5);
      }
      ReleaseDC(*(HWND *)((int)this + 0x18),hDC);
    }
    if (this_00 != (uint *)0x0) {
      FUN_005fbfa0(this_00);
      FUN_00618b60((undefined *)this_00);
    }
  }
  ExceptionList = local_c;
  return puVar6;
}

