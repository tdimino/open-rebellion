
void __thiscall FUN_00567f20(void *this,uint *param_1,void *param_2)

{
  void *this_00;
  int *piVar1;
  int iVar2;
  void *pvVar3;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b328;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f35b0(local_18,&DAT_006b120c);
  this_00 = param_2;
  local_4 = 0;
  FUN_005f3090(param_2,(int)local_18);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  piVar1 = FUN_004f26e0((int *)param_1);
  if (piVar1 == (int *)0x0) {
    FUN_005f35b0(local_18,s_<empty_SeedKey>_006ab13c);
    local_4 = 8;
    FUN_005f3090(this_00,(int)local_18);
  }
  else {
    iVar2 = FUN_00567790(this,param_1);
    if (iVar2 == 0) {
      FUN_005f35b0(local_18,s_<can_t_find_name,_type___006ab008);
      local_4 = 4;
      FUN_005f30d0(this_00,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(this_00,*param_1 >> 0x18);
      FUN_005f35b0(local_18,&DAT_006ab004);
      local_4 = 5;
      FUN_005f30d0(this_00,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f35b0(local_18,s__ID__006ab024);
      local_4 = 6;
      FUN_005f30d0(this_00,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(this_00,*param_1 & 0xffffff);
      FUN_005f35b0(local_18,&DAT_006aa5d0);
      local_4 = 7;
      FUN_005f30d0(this_00,(int)local_18);
    }
    else {
      param_2 = *(void **)(iVar2 + 0x30);
      pvVar3 = FUN_005f2fc0(local_18,&param_2);
      local_4 = 1;
      FUN_005f30d0(this_00,(int)pvVar3);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f35b0(local_18,s__ID__006ab024);
      local_4 = 2;
      FUN_005f30d0(this_00,(int)local_18);
      local_4 = 0xffffffff;
      FUN_005f2ff0(local_18);
      FUN_005f31f0(this_00,*param_1 & 0xffffff);
      FUN_005f35b0(local_18,&DAT_006aa5d0);
      local_4 = 3;
      FUN_005f30d0(this_00,(int)local_18);
    }
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

